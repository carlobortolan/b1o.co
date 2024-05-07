import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "b1o.co",
  description: "Chess-Inspired Algorithm for Image Ranking",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        {children}
        <footer className="w-full text-center border-t">
          <div className="flex justify-center items-center h-16 gap-4">
            <a
              href="https://github.com/carlobortolan/b1o.co"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-500 hover:text-gray-800"
            >
              GitHub
            </a>
            <a href="/about" className="text-gray-500 hover:text-gray-800">
              About
            </a>
            <a href="/imprint" className="text-gray-500 hover:text-gray-800">
              Imprint
            </a>
          </div>
        </footer>
      </body>
    </html>
  );
}
