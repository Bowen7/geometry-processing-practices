import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router";
import "./index.css";
import { Navigate } from "react-router";
import { PracticesSelect, practices } from "./practices-select";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <PracticesSelect />
      <Routes>
        <Route path="/" element={<Navigate to="/01" replace />} />
        {practices.map((practice) => (
          <Route
            key={practice.path}
            path={practice.path}
            element={practice.element}
          />
        ))}
      </Routes>
    </BrowserRouter>
  </StrictMode>
);
