import { useState } from "react";
import { Search } from "lucide-react";

interface InputBarProps {
  onSearch: (url: string) => void;
}

function InputBar({ onSearch }: InputBarProps) {
  const [url, setUrl] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (url.trim()) {
      onSearch(url.trim());
    }
  };

  return (
    <form className="input-bar" onSubmit={handleSubmit}>
      <div className="input-wrapper">
        <Search size={18} className="search-icon" />
        <input
          type="text"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          placeholder="输入 B站视频链接..."
          className="input-field"
        />
      </div>
      <button type="submit" className="btn btn-primary">
        解析
      </button>
    </form>
  );
}

export default InputBar;
