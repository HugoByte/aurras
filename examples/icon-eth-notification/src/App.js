import React, { useState } from "react";
import "./styles.css";
import {
  BrowserRouter as Router,
  Route,
  Routes,
} from "react-router-dom";
import Home from "./pages";
import Main from "./pages/MainPage";
import ModalApp from "./pages/Modal";

export default function App() {
  return (
    <Router>
      <Routes>
        <Route exact path="/" element={<Home />} />
        <Route path="/main" element={<Main />} />
        <Route path="/model" element={<ModalApp />} />
      </Routes>
    </Router>
  );
}
