import { PositionData } from '../ecs/components';

export interface AudioTrack {
  id: string;
  name: string;
  file: string;
  fallback: string;
  loop: boolean;
  loopStart?: number;
  loopEnd?: number | null;
  volume: number;
  description: string;
}

export interface AudioManifest {
  music: AudioTrack[];
  ambientSounds: AudioTrack[];
  soundEffects: AudioTrack[];
}

export interface AudioManager {
  loadManifest(manifest: AudioManifest): Promise<void>;
  playMusic(trackId: string): Promise<void>;
  playSoundEffect(effectId: string, position?: PositionData): void;
  crossfadeMusic(newTrackId: string, duration: number): Promise<void>;
  setVolume(type: 'master' | 'music' | 'sfx', volume: number): void;
  mute(): void;
  unmute(): void;
}
