import "../styles/globals.css";
import "windi.css";
import "antd/dist/antd.css";
import type { AppProps } from "next/app";
import BasicLayout from "layouts/BasicLayout";

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <BasicLayout>
      <Component {...pageProps} />
    </BasicLayout>
  );
}

export default MyApp;
