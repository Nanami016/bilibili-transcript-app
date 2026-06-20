import { invoke } from "@tauri-apps/api/core";

// 视频相关
export const parseVideo = (url: string) => invoke("parse_video", { url });
export const getVideoFormats = (url: string) => invoke("get_video_formats", { url });
export const downloadVideo = (url: string, formatId: string) =>
  invoke("download_video", { url, formatId });
export const downloadAudio = (url: string) => invoke("download_audio", { url });

// 转录相关
export const transcribe = (url: string) => invoke("transcribe", { url });
export const testWhisperConnection = () => invoke("test_whisper_connection");

// 收藏夹相关
export const getFavorites = () => invoke("get_favorites");
export const getFavoriteVideos = (mediaId: string) =>
  invoke("get_favorite_videos", { mediaId });

// 配置相关
export const getConfig = () => invoke("get_config");
export const updateConfig = (config: any) => invoke("update_config", { config });

// Cookie 相关
export const importCookie = (cookie: string) => invoke("import_cookie", { cookie });
export const getCookieStatus = () => invoke("get_cookie_status");
