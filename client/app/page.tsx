"use client";

import { ScoreboardItem } from "@/types/player";
import Image from "next/image";
import Link from "next/link";
import { useState, useEffect } from "react";

export default function Home() {
  const [left, setLeft] = useState<ScoreboardItem>();
  const [right, setRight] = useState<ScoreboardItem>();
  const [visitedPlayers, setVisitedPlayers] = useState<number[]>([1, 2, 3, 5]);
  const [leftIsLoading, setLeftIsLoading] = useState(false);
  const [rightIsLoading, setRightIsLoading] = useState(false);

  useEffect(() => {
    handleStart();
  }, []);

  const handleStart = () => {
    fetch("http://api.b1o.co/start", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
    })
      .then((response) => response.json())
      .then((data) => {
        setLeft(data[0]);
        setRight(data[1]);
      });
  };

  const handleClickLeft = () => {
    setRightIsLoading(true);
    fetch(`http://api.b1o.co/next?winner=${left?.id}&loser=${right?.id}`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ visited_ids: visitedPlayers }),
    })
      .then((response) => {
        if (!response.ok) {
          handleStart();
        } else {
          return response.json();
        }
      })
      .then((newPlayer) => {
        setRight(newPlayer);
        setVisitedPlayers((prevPlayers) => [...prevPlayers, newPlayer?.id]);
      })
      .finally(() => {
        setRightIsLoading(false);
      });
  };

  const handleClickRight = () => {
    setLeftIsLoading(true);
    fetch(`http://api.b1o.co/next?winner=${right?.id}&loser=${left?.id}`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ visited_ids: visitedPlayers ?? [] }),
    })
      .then((response) => {
        if (!response.ok) {
          handleStart();
        } else {
          return response.json();
        }
      })
      .then((newPlayer) => {
        setLeft(newPlayer);
        setVisitedPlayers((prevPlayers) => [...prevPlayers, newPlayer?.id]);
      })
      .finally(() => {
        setLeftIsLoading(false);
      });
  };

  return !left || !right ? (
    <main className="flex min-h-screen flex-col items-center justify-between p-6 pt-24 lg:p-24 lg:pt-16">
      <div className="z-10 w-full max-w-10xl items-center justify-between font-mono text-sm lg:flex mb-5">
        <Link
          href="/scoreboard"
          className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30"
        >
          Go to scoreboard
        </Link>
      </div>
      <div className="mb-32 grid text-center lg:w-full lg:max-w-5xl lg:grid-cols-2 lg:text-left">
        Loading...
      </div>
    </main>
  ) : (
    <main className="flex min-h-screen flex-col items-center justify-between p-6 pt-24 lg:p-24 lg:pt-16">
      <div className="z-10 w-full max-w-10xl items-center justify-between font-mono text-sm lg:flex">
        <Link
          href="/scoreboard"
          className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30"
        >
          Go to scoreboard
        </Link>
      </div>
      <div className="mb-32 grid text-center lg:w-full lg:max-w-5xl lg:grid-cols-2 lg:text-left">
        <div
          key={left.id}
          onClick={() => {
            if (!leftIsLoading) handleClickLeft();
          }}
        >
          {leftIsLoading ? (
            <div>Loading...</div>
          ) : (
            <div className="w-full h-full group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30">
              <h2 className="mb-3 text-2xl font-semibold">
                Upvote {left?.name}
                <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
                  -&gt;
                </span>
              </h2>
              <Image
                className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70]"
                src={left.image_url}
                alt={left.name}
                width={500}
                height={500}
                priority
              />
            </div>
          )}
        </div>
        <div
          key={right.id}
          onClick={() => {
            if (!rightIsLoading) handleClickRight();
          }}
        >
          {rightIsLoading ? (
            <div>Loading...</div>
          ) : (
            <div className="w-full h-full group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30">
              <h2 className="mb-3 text-2xl font-semibold">
                Upvote {right?.name}
                <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
                  -&gt;
                </span>
              </h2>
              <Image
                className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70]"
                src={right.image_url}
                alt={right.name}
                width={500}
                height={500}
                priority
              />
            </div>
          )}
        </div>
      </div>
    </main>
  );
}
