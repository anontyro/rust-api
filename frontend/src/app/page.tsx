"use client";
import Image from "next/image";
import React from "react";

export default function Home() {
  const [isLoading, setIsLoading] = React.useState<boolean>(true);
  const [response, setResponse] = React.useState<any>(null);

  React.useEffect(() => {
    const getOctopusEndpoint = async () => {
      setIsLoading(true);
      const url = "http://localhost:5001/api/octopus/about";
      console.log("url", url);
      const response = await fetch(url);

      if (response.ok) {
        const json = await response.json();
        console.log("json", json);
        setResponse(json);
      }
    };

    getOctopusEndpoint();
  }, []);

  return (
    <div>
      <h1>About Page</h1>
      <p>{response === null ? "No content" : response}</p>
    </div>
  );
}
