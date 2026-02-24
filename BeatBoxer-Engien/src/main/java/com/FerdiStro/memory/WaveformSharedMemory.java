package com.FerdiStro.memory;

import org.deepsymmetry.beatlink.Util;
import org.deepsymmetry.beatlink.data.WaveformDetail;

import java.io.IOException;
import java.io.RandomAccessFile;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;

public class WaveformSharedMemory {

    private static final int MAX_WAVEFORM_SIZE = 150_000;

    /**
     * Fast grid Data 32 byte long
     */
    private static final int OFFSET_ACTIVE_BUFFER = 0;
    private static final int OFFSET_BPM = 4;
    private static final int OFFSET_PLAY_HEAD = 8;
    private static final int HEADER_SIZE = 16;


    // ~50 Min on 200 BPM. Way overhead for mixing. maybe turn down in future
    private static final int MAX_BEATS = 10_000;
    private static final int BEAT_ARRAY_BYTES = MAX_BEATS * 4;
    private static final int TIME_ARRAY_BYTES = MAX_BEATS * 8;
    private static final int TRACK_BUFFER_SIZE = 4 + 4 + MAX_WAVEFORM_SIZE + 4 + BEAT_ARRAY_BYTES + TIME_ARRAY_BYTES;
    private static final int BUFFER_1_OFFSET = HEADER_SIZE + TRACK_BUFFER_SIZE;
    private static final int TOTAL_FILE_SIZE = HEADER_SIZE + (TRACK_BUFFER_SIZE * 2);

    /**
     * Buffer Offset
     */
    private static final int BUFFER_0_OFFSET = HEADER_SIZE;
    private static final String FILE_PATH = "_player_wave_form.bin";

    private final MappedByteBuffer mappedBuffer;
    int cachedBeatCount = 0;
    private byte currentActiveBuffer = 0;
    /**
     * Local cache for sync
     */
    private int cachedTrackId = 0;
    private byte[] cachedWaveformData = new byte[0];
    private int[] cachedBeatWithinBar = new int[0];
    private long[] cachedTimeWithinTrack = new long[0];

    public WaveformSharedMemory(int playerPrefix) throws IOException {
        String filePath = playerPrefix + FILE_PATH;
        try (RandomAccessFile raf = new RandomAccessFile(filePath, "rw")) {
            raf.setLength(TOTAL_FILE_SIZE);
            mappedBuffer = raf.getChannel().map(FileChannel.MapMode.READ_WRITE, 0, TOTAL_FILE_SIZE);
            mappedBuffer.order(ByteOrder.LITTLE_ENDIAN);
            mappedBuffer.put(0, currentActiveBuffer);
        }
    }

    public void updatePlayHead(long playHeadMs, int bpm) {
        long playHeadIndex = (playHeadMs * 75) / 1000;
        mappedBuffer.putFloat(OFFSET_BPM, bpm);
        mappedBuffer.putLong(OFFSET_PLAY_HEAD, playHeadIndex);
    }

    public synchronized void updateBeatGrid(ByteBuffer rawData) {
        final byte[] gridBytes = new byte[rawData.remaining()];
        rawData.get(gridBytes);

        int beatCount = Math.max(0, (gridBytes.length - 20) / 16);
        int[] beatWithinBarValues = new int[beatCount];
        long[] timeWithinTrackValues = new long[beatCount];

        for (int beatNumber = 0; beatNumber < beatCount; beatNumber++) {
            final int base = 20 + beatNumber * 16;
            beatWithinBarValues[beatNumber] = (int) Util.bytesToNumberLittleEndian(gridBytes, base, 2);
            timeWithinTrackValues[beatNumber] = Util.bytesToNumberLittleEndian(gridBytes, base + 4, 4);
        }

        this.cachedBeatWithinBar = beatWithinBarValues;
        this.cachedTimeWithinTrack = timeWithinTrackValues;
        this.cachedBeatCount = beatCount;

        commitToInactiveBufferAndSwap();
    }

    public synchronized void updateWaveFrom(WaveformDetail waveformDetail) {
        if (waveformDetail != null) {
            this.cachedTrackId = waveformDetail.dataReference.rekordboxId;

            ByteBuffer data = waveformDetail.getData().duplicate();
            data.rewind();

            int len = Math.min(data.remaining(), MAX_WAVEFORM_SIZE);
            this.cachedWaveformData = new byte[len];
            data.get(this.cachedWaveformData);
            commitToInactiveBufferAndSwap();
        }
    }

    private void commitToInactiveBufferAndSwap() {
        byte nextBuffer = (byte) (1 - currentActiveBuffer);
        int bufferStart = (nextBuffer == 0) ? BUFFER_0_OFFSET : BUFFER_1_OFFSET;


        ByteBuffer writeBuffer = mappedBuffer.duplicate();
        writeBuffer.order(ByteOrder.LITTLE_ENDIAN);


        //WaveForm
        writeBuffer.position(bufferStart);
        writeBuffer.putInt(cachedTrackId);
        writeBuffer.putInt(cachedWaveformData.length);
        writeBuffer.put(cachedWaveformData);


        //BeatGrid
        int beatStart = bufferStart + 8 + MAX_WAVEFORM_SIZE;
        int beatLen = Math.min(cachedBeatWithinBar.length, MAX_BEATS);

        if (beatLen != this.cachedBeatCount) {
            return;
        }

        writeBuffer.position(beatStart);
        writeBuffer.putInt(beatLen);
        for (int i = 0; i < beatLen; i++) {
            writeBuffer.putInt(cachedBeatWithinBar[i]);
        }

        int timeStart = beatStart + 4 + BEAT_ARRAY_BYTES;
        writeBuffer.position(timeStart);
        for (int i = 0; i < beatLen; i++) {
            writeBuffer.putLong(cachedTimeWithinTrack[i]);
        }

        //Change offset
        writeBuffer.put(OFFSET_ACTIVE_BUFFER, nextBuffer);
        currentActiveBuffer = nextBuffer;
    }
}