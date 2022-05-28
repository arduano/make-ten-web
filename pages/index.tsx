import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";
import styles from "../styles/Home.module.css";

import * as test from "../calculator/pkg/calculator_bg";
import { useEffect, useMemo, useState } from "react";
import Box from "@mui/material/Box";
import { TextField } from "@mui/material";

const Home: NextPage = () => {
  const [text, setText] = useState("");

  const results = useMemo(() => {
    let numbers = text.split("").map(Number);
    const vals = test.generate_solutions(new Int32Array(numbers)) as string[];
    return vals;
  }, [text]);

  return (
    <Box
      minHeight={"100vh"}
      display="flex"
      flexDirection="column"
      alignItems="center"
      pt={4}
      px={4}
    >
      <TextField
        inputProps={{
          style: { textAlign: "center", fontSize: 32 },
        }}
        InputLabelProps={{
          style: { textAlign: "center", fontSize: 32 },
        }}
        label="Digits"
        type="number"
        variant="standard"
        value={text}
        onChange={(e) => {
          let value = e.target.value;
          if (!value.match(/[^0-9]/g) && value.length <= 6) {
            setText(value);
          }
        }}
        onKeyDown={(e) => {
          // Apparently you can type "e" as in exponent into the box by default
          // so that is prevented here
          if (e.key === "e") {
            e.preventDefault();
          }
        }}
      />
      <Box py={3} display="flex" flexDirection="column" alignItems="center">
        {results.map((result) => (
          <Box key={result}>{result}</Box>
        ))}
      </Box>
    </Box>
  );
};

export default Home;
