import type { DocumentContext } from "next/document";
import Document, { Head, Html, Main, NextScript } from "next/document";

class MyDocument extends Document {
  static async getInitialProps(ctx: DocumentContext) {
    const initialProps = await Document.getInitialProps(ctx);
    return { ...initialProps };
  }

  render() {
    return (
      <Html lang="en">
        <Head>
          <link
            rel="apple-touch-icon"
            type="image/png"
            href="favicons/apple-touch-icon.png"
            sizes="180x180"
          />
          <link rel="mask-icon" href="favicons/stencil.svg" color="#0098fe" />
          <link
            rel="icon"
            type="image/png"
            href="favicons/favicon-32x32.png"
            sizes="32x32"
          />
          <link
            rel="icon"
            type="image/png"
            href="favicons/favicon-16x16.png"
            sizes="16x16"
          />
          <link rel="icon" href="/favicon.ico" />

          {/* Fonts */}
          <link rel="preconnect" href="https://fonts.gstatic.com" />
          <link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap"
          />

          <link
            rel="stylesheet"
            href="https://fonts.googleapis.com/icon?family=Material+Icons"
          />

          {/* <meta name="viewport" content="width=device-width, initial-scale=1" /> */}
        </Head>
        <body>
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}

export default MyDocument;
