import "../styles/globals.css";
import type { AppProps } from "next/app";
import { createTheme, ThemeProvider, CssBaseline } from "@mui/material";
import Head from "next/head";

const theme = createTheme({
  palette: {
    mode: "dark",
  },
});

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Head>
        <title>Make Ten</title>
      </Head>
      <Component {...pageProps} />
    </ThemeProvider>
  );
}

export default MyApp;
