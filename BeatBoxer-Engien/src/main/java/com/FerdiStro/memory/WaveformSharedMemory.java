package com.FerdiStro.memory;

import org.deepsymmetry.beatlink.data.WaveformDetail;

import java.io.IOException;
import java.io.RandomAccessFile;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;

public class WaveformSharedMemory {

    private static final int MAX_WAVEFORM_SIZE = 150_000;
    private static final int HEADER_SIZE = 4;
    private static final int BUFFER_0_OFFSET = HEADER_SIZE;
    private static final int BUFFER_1_OFFSET = HEADER_SIZE + 8 + MAX_WAVEFORM_SIZE; // 8 Bytes für trackId & length
    private static final int TOTAL_FILE_SIZE = BUFFER_1_OFFSET + 8 + MAX_WAVEFORM_SIZE;

    private static final String FILE_PATH = "_player_wave_form.bin";

    private final MappedByteBuffer mappedBuffer;
    private byte currentActiveBuffer = 0;

    public WaveformSharedMemory(int playerPrefix) throws IOException {
        String filePath = playerPrefix + FILE_PATH;
        try (RandomAccessFile raf = new RandomAccessFile(filePath, "rw")) {
            raf.setLength(TOTAL_FILE_SIZE);
            mappedBuffer = raf.getChannel().map(FileChannel.MapMode.READ_WRITE, 0, TOTAL_FILE_SIZE);
            mappedBuffer.order(ByteOrder.LITTLE_ENDIAN);
            mappedBuffer.put(0, currentActiveBuffer);
        }
    }


    public void updateWaveFrom(WaveformDetail waveformDetail) {
        if (waveformDetail != null) {
            writeUpdateToFile(waveformDetail.dataReference.rekordboxId, waveformDetail.getData());
        }
    }

    private void writeUpdateToFile(int trackId, ByteBuffer sourceData) {
        byte nextBuffer = (byte) (1 - currentActiveBuffer);
        int bufferStart = (nextBuffer == 0) ? BUFFER_0_OFFSET : BUFFER_1_OFFSET;

        ByteBuffer dataToCopy = sourceData.duplicate();
        dataToCopy.rewind();

        int len = Math.min(dataToCopy.remaining(), MAX_WAVEFORM_SIZE);
        mappedBuffer.position(bufferStart);
        mappedBuffer.putInt(trackId);
        mappedBuffer.putInt(len);

        if (dataToCopy.remaining() > len) {
            dataToCopy.limit(len);
        }

        mappedBuffer.put(dataToCopy);
        mappedBuffer.put(0, nextBuffer);
        currentActiveBuffer = nextBuffer;
    }


}