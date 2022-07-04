import React from "react";
import Head from "next/head";
import { useRouter } from "next/router";
import Header from "components/Header";

import Footer from "components/Footer";

export default function BasicLayout({ children, ...rest }: any) {
  return (
    <div>
      <Head>
        <title>Scroll Explorer</title>
        <meta name="description" content="Scroll Explorer" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Header />
      <main className="min-h-400px">{children}</main>
      <Footer />
    </div>
  );
}
