import { Routes, Route, Navigate } from 'react-router-dom';
import { useState } from 'react';
import Layout from './components/Layout';
import Login from './pages/Login';
import ChallengeList from './pages/ChallengeList';
import ChallengeEditor from './pages/ChallengeEditor';
import { getToken } from './api/client';

function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const token = getToken();
  if (!token) {
    return <Navigate to="/login" replace />;
  }
  return <>{children}</>;
}

export default function App() {
  const [, setIsAuthenticated] = useState(!!getToken());

  return (
    <Routes>
      <Route path="/login" element={<Login onLogin={() => setIsAuthenticated(true)} />} />
      <Route
        path="/"
        element={
          <ProtectedRoute>
            <Layout />
          </ProtectedRoute>
        }
      >
        <Route index element={<ChallengeList />} />
        <Route path="challenges/new" element={<ChallengeEditor />} />
        <Route path="challenges/:id" element={<ChallengeEditor />} />
      </Route>
    </Routes>
  );
}
