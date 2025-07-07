using System;

namespace PowerPointController
{
    internal class Program
    {
        static void Main(string[] args)
        {
            var controller = new Controller();
            controller.Start(args[0]);
            Console.WriteLine("PowerPoint bridge gestart. Druk op Enter om te stoppen...");
            Console.ReadLine();
            controller.Stop();
        }
    }
}