import {
  QueryClient,
  QueryClientProvider,
} from '@tanstack/react-query'
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter, Navigate, Route, Routes } from 'react-router'

import { practices } from './practices'
import { PracticesSelect } from './practices-select'
import './index.css'

const queryClient = new QueryClient()

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <QueryClientProvider client={queryClient}>
        <Routes>
          <Route path="/" element={<Navigate to="/01" replace />} />
          {practices.map(practice => (
            <Route
              key={practice.path}
              path={practice.path}
              element={practice.element}
            />
          ))}
        </Routes>
        <PracticesSelect />
      </QueryClientProvider>
    </BrowserRouter>
  </StrictMode>,
)
