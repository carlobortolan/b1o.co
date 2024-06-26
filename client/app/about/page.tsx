import Link from "next/link";
import Image from "next/image";

export default function About() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-6 pt-24 pb-24 2xl:p-24 2xl:pt-16">
      <div className="w-full max-w-10xl items-top justify-between font-mono lg:col-3 text-sm lg:flex mb-5">
        <Link
          href="/"
          className="z-10 fixed left-0 top-0 flex w-full xl:w-2/5 2xl:w-1/4 justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:h-full lg:static lg:w-80px lg:rounded-xl lg:border lg:bg-gray-200 md:p-4 lg:p-4 lg:dark:bg-zinc-800/30"
        >
          Back to Home
        </Link>
        <div className="relative p-6 2xl:p-32 italic dark:drop-shadow-[0_0_0.3rem_#ffffff70]">
          &quot;The measurement of the rating of an individual might well be
          compared with the measurement of the position of a cork bobbing up and
          down on the surface of agitated water with a yardstick tied to a rope
          and which is swaying in the wind.&quot;
          <p className="text-sm font-bold mt-4">
            — Arpad Elo, creator of the Elo rating system
          </p>
        </div>
        <Image
          className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70]"
          src="/banner.jpeg"
          alt="b1o.co logo"
          width={800}
          height={400}
          priority
        />
      </div>
      <div className="mb-0 grid gap-8 grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        <h1 className="text-4xl mb-4">About b1o.co</h1>
        <p className="text-justify">
          b1o.co is a platform where users can vote for their favorite images.
          Inspired from the{" "}
          <a
            href="https://en.wikipedia.org/wiki/Elo_rating_system#Theory"
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-500 hover:underline"
          >
            Elo rating system
          </a>{" "}
          used in chess, each image is assigned a rating. When a user votes for
          an image, it&apos;s considered a &quot;win&quot; for that image
          against the one it was compared to. The winning image gains points
          depending on the difference in ratings between the two images. If the
          winning image had a lower rating initially, it would gain more points.
          Conversely, if it had a higher rating, it would gain fewer points. The
          losing image loses the same number of points that the winning image
          gains.
        </p>
        <p className="text-justify">
          A{" "}
          <a
            href="https://en.wikipedia.org/wiki/Convolutional_neural_network"
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-500 hover:underline"
          >
            Convolutional Neural Network (CNN)
          </a>{" "}
          classifies the images, while a{" "}
          <a
            href="https://en.wikipedia.org/wiki/Multilayer_perceptron"
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-500 hover:underline"
          >
            Multilayer Perceptron (MLP)
          </a>{" "}
          is then used to analyze voting patterns and image characteristics. As
          always, any use of AI is done with the utmost respect for user privacy
          and without the use of cookies or any form of tracking. <br /> The
          b1o.co API is written in Rust and uses the Actix web framework to
          provide a fast experience for users. If you find any issues or have
          suggestions for improvement, please submit them{" "}
          <a
            href="https://github.com/carlobortolan/b1o.co/issues"
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-500 hover:underline"
          >
            here
          </a>
          .
        </p>
        <p className="text-justify">
          In terms of data storage, no personal data is stored. The images you
          see on b1o.co are not stored on b1o.co&apos;s servers. Instead, only
          the references to images that are hosted elsewhere are stored.
          Furthermore, to protect your privacy, no cookies are used. This means
          that your activity and data can&apos;t be tracked or collected.
        </p>
      </div>
    </main>
  );
}
