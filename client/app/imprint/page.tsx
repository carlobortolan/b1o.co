import Link from "next/link";

export default function Imprint() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-6 pt-24 pb-24 lg:p-24 lg:pt-16">
      <div className="z-10 w-full max-w-10xl items-top justify-between font-mono lg:col-2 text-sm lg:flex mb-5">
        <Link
          href="/"
          className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit h-14 lg:static lg:w-auto lg:rounded-xl lg:border lg:bg-gray-200 md:p-4 lg:p-4 lg:dark:bg-zinc-800/30"
        >
          Back to Home
        </Link>
        <div className="mb-0 md:px-16 lg:px-24">
          <h1 className="text-4xl mb-4">IMPRINT</h1>
          <p className="text-2xl">2me.ai is operated by:</p>
          <p>
            <code>
              Carlo Bortolan
              <br />
              Heimstättenstraße 6
              <br />
              Munich, 80805
              <br />
              Germany
            </code>
          </p>
          <br />
          <p>
            <p className="text-2xl">Contact:</p>
            <code>
              Email: carlobortolan@gmail.com
              <br />
              Phone: +49 152 25237383
            </code>
          </p>
        </div>
      </div>
    </main>
  );
}
