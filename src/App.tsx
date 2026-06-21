import { BrowserRouter, Routes, Route } from "react-router-dom";
import Layout from "./components/Layout";
import Home from "./pages/Home";
import Favorite from "./pages/Favorite";
import VideoDownload from "./pages/VideoDownload";
import AudioDownload from "./pages/AudioDownload";
import AIAnalysis from "./pages/AIAnalysis";
import AudioTranscribe from "./pages/AudioTranscribe";
import Logs from "./pages/Logs";
import Settings from "./pages/Settings";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="favorite" element={<Favorite />} />
          <Route path="tasks/video" element={<VideoDownload />} />
          <Route path="tasks/audio" element={<AudioDownload />} />
          <Route path="tasks/ai" element={<AIAnalysis />} />
          <Route path="tasks/transcribe" element={<AudioTranscribe />} />
          <Route path="logs" element={<Logs />} />
          <Route path="settings" element={<Settings />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
