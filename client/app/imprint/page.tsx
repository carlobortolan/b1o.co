import Link from "next/link";
import Image from "next/image";

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
          <p className="text-2xl">b1o.co is operated by:</p>
          <Image
            className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70]"
            src="/imprint1.png"
            alt="imprint1"
            width={200}
            height={250}
            priority
          />
          <br />
          <p className="text-2xl">Contact:</p>
          <p>
            <Image
              className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70]"
              src="/imprint2.png"
              alt="imprint2"
              width={300}
              height={300}
              priority
            />
          </p>
        </div>
      </div>
    </main>
  );
}
