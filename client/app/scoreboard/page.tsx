"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { ScoreboardItem } from "@/types/player";

export default function Scoreboard() {
  const [data, setData] = useState<ScoreboardItem[]>([]);
  const [page, setPage] = useState(1);
  const [loading, setLoading] = useState(false);
  const [hasMore, setHasMore] = useState(true);

  const loadMore = () => {
    console.log("loadMore called");
    if (!loading && hasMore) {
      setLoading(true);
      fetch(`http://localhost:8080/scoreboard?page=${page}&limit=10`)
        .then((response) => response.json())
        .then((newData) => {
          setData((prevData) => [...prevData, ...newData]);
          setLoading(false);
          if (newData.length === 0) {
            setHasMore(false);
          } else {
            setPage((prevPage) => prevPage + 1);
          }
        });
    }
  };

  useEffect(() => {
    console.log("useEffect called");
    // loadMore();
  }, []);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-6 pt-24 lg:p-24 lg:pt-16">
      <div className="z-10 w-full max-w-10xl items-center justify-between font-mono text-sm lg:flex mb-5">
        <Link
          href="/"
          className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30"
        >
          Return to selector
        </Link>
      </div>
      <div className="mb-0 grid gap-4 grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {data.map((item) => (
          <div key={item.id} className="border p-4 rounded shadow">
            <img
              src={item.image_url}
              alt={item.name}
              className="w-full h-64 object-cover rounded"
              style={{ width: "300px", height: "450px", objectFit: "cover" }}
            />
            <h2 className="mt-4 text-xl font-bold">{item.name}</h2>
            <div className="flex items-center mt-2">
              <button className="mr-1" disabled>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  className="h-6 w-6 text-green-500"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 15l7-7 7 7"
                  />
                </svg>
              </button>
              <span>{item.upvotes}</span>
              <button className="ml-4 mr-1" disabled>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  className="h-6 w-6 text-red-500"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M19 9l-7 7-7-7"
                  />
                </svg>
              </button>
              <span>{item.downvotes}</span>
              <div className="ml-auto text-sm font-light text-right">
                Rating:{" "}
                <span className="font-black">
                  {Math.round(Number(item.rating))}
                </span>{" "}
              </div>
            </div>
          </div>
        ))}
        {loading && (
          <div className="col-span-full flex justify-center items-center">
            Loading...
          </div>
        )}
        {!hasMore && (
          <div className="col-span-full text-center text-gray-500">
            You&apos;ve reached the end!
          </div>
        )}
        {hasMore && !loading && (
          <div className="col-span-full text-center">
            <button
              onClick={loadMore}
              className="px-4 py-2 bg-blue-500 text-white rounded"
            >
              Load more
            </button>
          </div>
        )}
      </div>
    </main>
  );
}
