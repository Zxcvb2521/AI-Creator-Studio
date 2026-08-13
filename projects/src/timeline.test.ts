import { describe, it, expect } from 'vitest';
import { addAssetToTimeline, removeTimelineItem, moveTimelineItem } from './timeline';
import { createProject } from './model';
import type { ProjectAsset } from './model';

describe('timeline', () => {
  describe('addAssetToTimeline', () => {
    it('adds video asset to video track', () => {
      const project = createProject('Test Project');
      const videoAsset: ProjectAsset = {
        id: 'video-1',
        kind: 'video',
        name: 'Test Video',
        path: '/path/to/video.mp4',
        duration: 10,
        createdAt: new Date().toISOString(),
      };
      
      const updated = addAssetToTimeline(project, videoAsset);
      const videoTrack = updated.tracks.find(t => t.kind === 'video');
      
      expect(videoTrack?.items).toHaveLength(1);
      expect(videoTrack?.items[0].assetId).toBe('video-1');
      expect(videoTrack?.items[0].duration).toBe(10);
      expect(videoTrack?.items[0].start).toBe(0);
    });

    it('adds image asset to video track', () => {
      const project = createProject('Test Project');
      const imageAsset: ProjectAsset = {
        id: 'image-1',
        kind: 'image',
        name: 'Test Image',
        path: '/path/to/image.png',
        createdAt: new Date().toISOString(),
      };
      
      const updated = addAssetToTimeline(project, imageAsset);
      const videoTrack = updated.tracks.find(t => t.kind === 'video');
      
      expect(videoTrack?.items).toHaveLength(1);
      expect(videoTrack?.items[0].assetId).toBe('image-1');
      expect(videoTrack?.items[0].duration).toBe(5); // Default duration
    });

    it('adds voice asset to voice track', () => {
      const project = createProject('Test Project');
      const voiceAsset: ProjectAsset = {
        id: 'voice-1',
        kind: 'voice',
        name: 'Test Voice',
        path: '/path/to/voice.wav',
        duration: 3,
        createdAt: new Date().toISOString(),
      };
      
      const updated = addAssetToTimeline(project, voiceAsset);
      const voiceTrack = updated.tracks.find(t => t.kind === 'voice');
      
      expect(voiceTrack?.items).toHaveLength(1);
      expect(voiceTrack?.items[0].assetId).toBe('voice-1');
    });

    it('adds audio asset to audio track', () => {
      const project = createProject('Test Project');
      const audioAsset: ProjectAsset = {
        id: 'audio-1',
        kind: 'audio',
        name: 'Test Music',
        path: '/path/to/music.mp3',
        duration: 30,
        createdAt: new Date().toISOString(),
      };
      
      const updated = addAssetToTimeline(project, audioAsset);
      const audioTrack = updated.tracks.find(t => t.kind === 'audio');
      
      expect(audioTrack?.items).toHaveLength(1);
      expect(audioTrack?.items[0].assetId).toBe('audio-1');
    });

    it('appends item after existing items on the same track', () => {
      const project = createProject('Test Project');
      const videoAsset1: ProjectAsset = {
        id: 'video-1',
        kind: 'video',
        name: 'First Video',
        path: '/path/to/video1.mp4',
        duration: 10,
        createdAt: new Date().toISOString(),
      };
      const videoAsset2: ProjectAsset = {
        id: 'video-2',
        kind: 'video',
        name: 'Second Video',
        path: '/path/to/video2.mp4',
        duration: 5,
        createdAt: new Date().toISOString(),
      };
      
      const withFirst = addAssetToTimeline(project, videoAsset1);
      const withSecond = addAssetToTimeline(withFirst, videoAsset2);
      
      const videoTrack = withSecond.tracks.find(t => t.kind === 'video');
      expect(videoTrack?.items).toHaveLength(2);
      expect(videoTrack?.items[0].start).toBe(0);
      expect(videoTrack?.items[1].start).toBe(10); // After first video
    });

    it('returns unchanged project if no compatible track', () => {
      const project = createProject('Test Project');
      // Remove all tracks to simulate no compatible track
      const brokenProject = { ...project, tracks: [] };
      
      const asset: ProjectAsset = {
        id: 'video-1',
        kind: 'video',
        name: 'Test Video',
        path: '/path/to/video.mp4',
        createdAt: new Date().toISOString(),
      };
      
      const updated = addAssetToTimeline(brokenProject, asset);
      expect(updated).toBe(brokenProject);
    });
  });

  describe('removeTimelineItem', () => {
    it('removes item from track', () => {
      const project = createProject('Test Project');
      const videoAsset: ProjectAsset = {
        id: 'video-1',
        kind: 'video',
        name: 'Test Video',
        path: '/path/to/video.mp4',
        duration: 10,
        createdAt: new Date().toISOString(),
      };
      
      const withAsset = addAssetToTimeline(project, videoAsset);
      const itemId = withAsset.tracks.find(t => t.kind === 'video')?.items[0].id!;
      
      const updated = removeTimelineItem(withAsset, itemId);
      const videoTrack = updated.tracks.find(t => t.kind === 'video');
      
      expect(videoTrack?.items).toHaveLength(0);
    });

    it('updates updatedAt', () => {
      const project = createProject('Test Project');
      const videoAsset: ProjectAsset = {
        id: 'video-1',
        kind: 'video',
        name: 'Test Video',
        path: '/path/to/video.mp4',
        duration: 10,
        createdAt: new Date().toISOString(),
      };
      
      const withAsset = addAssetToTimeline(project, videoAsset);
      const itemId = withAsset.tracks.find(t => t.kind === 'video')?.items[0].id!;
      
      const updated = removeTimelineItem(withAsset, itemId);
      expect(updated.updatedAt).not.toBe(withAsset.updatedAt);
    });
  });

  describe('moveTimelineItem', () => {
    it('moves item to new start position', () => {
      const project = createProject('Test Project');
      const videoAsset: ProjectAsset = {
        id: 'video-1',
        kind: 'video',
        name: 'Test Video',
        path: '/path/to/video.mp4',
        duration: 10,
        createdAt: new Date().toISOString(),
      };
      
      const withAsset = addAssetToTimeline(project, videoAsset);
      const itemId = withAsset.tracks.find(t => t.kind === 'video')?.items[0].id!;
      
      const updated = moveTimelineItem(withAsset, itemId, 20);
      const videoTrack = updated.tracks.find(t => t.kind === 'video');
      
      expect(videoTrack?.items[0].start).toBe(20);
    });

    it('prevents negative start positions', () => {
      const project = createProject('Test Project');
      const videoAsset: ProjectAsset = {
        id: 'video-1',
        kind: 'video',
        name: 'Test Video',
        path: '/path/to/video.mp4',
        duration: 10,
        createdAt: new Date().toISOString(),
      };
      
      const withAsset = addAssetToTimeline(project, videoAsset);
      const itemId = withAsset.tracks.find(t => t.kind === 'video')?.items[0].id!;
      
      const updated = moveTimelineItem(withAsset, itemId, -10);
      const videoTrack = updated.tracks.find(t => t.kind === 'video');
      
      expect(videoTrack?.items[0].start).toBe(0);
    });

    it('updates updatedAt', () => {
      const project = createProject('Test Project');
      const videoAsset: ProjectAsset = {
        id: 'video-1',
        kind: 'video',
        name: 'Test Video',
        path: '/path/to/video.mp4',
        duration: 10,
        createdAt: new Date().toISOString(),
      };
      
      const withAsset = addAssetToTimeline(project, videoAsset);
      const itemId = withAsset.tracks.find(t => t.kind === 'video')?.items[0].id!;
      
      const updated = moveTimelineItem(withAsset, itemId, 20);
      expect(updated.updatedAt).not.toBe(withAsset.updatedAt);
    });
  });
});
