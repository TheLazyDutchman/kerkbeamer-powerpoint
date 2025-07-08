using Microsoft.Office.Interop.PowerPoint;
using WebSocketSharp.Server;
using WebSocketSharp;
using Microsoft.Office.Core;
using System.Runtime.InteropServices;
using System;

namespace PowerPointController
{

    public class Controller
    {
        private Application _app;
        private Presentation _presentation;
        private SlideShowWindow _slideshow;
        private WebSocketServer _wss;
 
        [DllImport("user32.dll")]
        static extern bool MoveWindow(IntPtr hWnd, int X, int Y, int nWidth, int nHeight, bool bRepaint);

        public void Start(string path)
        {

            // Start WebSocket server
            _wss = new WebSocketServer(8181);
            _wss.AddWebSocketService<PowerPointWebSocket>("/ppt");
            _wss.Start();

            // Start PowerPoint
            _app = new Application();
            _app.Visible = MsoTriState.msoTrue;

            // Open presentatie
            _presentation = _app.Presentations.Open(path,
                WithWindow: MsoTriState.msoTrue);

            // Start slideshow en registreer event
            var settings = _presentation.SlideShowSettings;
            settings.ShowWithNarration = MsoTriState.msoFalse;
            settings.ShowWithAnimation = MsoTriState.msoTrue;
            _slideshow = settings.Run();
            _slideshow.View.GotoSlide(1); // naar begin

            MoveWindow((IntPtr)_slideshow.HWND, -2000, -2000, 800, 600, false);

            _app.SlideShowNextSlide += OnNextSlide;
        }

        private void OnNextSlide(SlideShowWindow Wn)
        {
            int currentSlide = Wn.View.CurrentShowPosition;
            PowerPointWebSocket.BroadcastSlideChanged(currentSlide);
        }

        public void Stop()
        {
            _presentation?.Close();
            _app?.Quit();
            _wss?.Stop();
        }
    }

    public class PowerPointWebSocket : WebSocketBehavior
    {
        private static WebSocketSessionManager _sessions;

        protected override void OnOpen()
        {
            _sessions = Sessions;
        }

        protected override void OnMessage(MessageEventArgs e)
        {
            if (e.Data == "next")
            {
                var app = new Application();
                var view = app?.SlideShowWindows?[1]?.View;
                view?.Next();
            }
            else if (e.Data.StartsWith("goto:"))
            {
                int slideNum = int.Parse(e.Data.Split(':')[1]);
                var app = new Application();
                var view = app?.SlideShowWindows?[1]?.View;
                view?.GotoSlide(slideNum);
            }
        }

        public static void BroadcastSlideChanged(int slideNumber)
        {
            _sessions?.Broadcast($"slide:{slideNumber}");
        }
    }
}
