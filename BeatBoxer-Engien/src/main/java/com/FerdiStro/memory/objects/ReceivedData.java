package com.FerdiStro.memory.objects;

import lombok.Getter;
import lombok.ToString;

import java.nio.MappedByteBuffer;
import java.nio.charset.StandardCharsets;

@ToString
public class ReceivedData {
    private static final int POSITION_INCREASE_BPM = 8;
    private static final int POSITION_DECREASE_BPM = 9;
    private static final int POSITION_BECOME_MASTER = 10;

    private static final int POSITION_SMALL_COUNTER = 14;
    private static final int POSITION_ADD_SOUND_ON_SMALL_BEAT = 15;
    private static final int POSITION_OFF_PATH = 16;
    private static final int POSITION_PATH_LEN = 256;

    @Getter
    private final boolean increaseBpm;
    @Getter
    private final boolean decreaseBpm;
    @Getter
    private final boolean becomeMaster;
    @Getter
    private final int smallCounter;
    @Getter
    private final boolean addSoundOnSmallBeat;

    @Getter
    private final String selectedSoundPath;

    public ReceivedData(MappedByteBuffer buffer) {
        this.increaseBpm = byteToBoolean(buffer.get(ReceivedData.POSITION_INCREASE_BPM));
        this.decreaseBpm = byteToBoolean(buffer.get(ReceivedData.POSITION_DECREASE_BPM));
        this.becomeMaster = byteToBoolean(buffer.get(ReceivedData.POSITION_BECOME_MASTER));

        this.smallCounter = Byte.toUnsignedInt(buffer.get(ReceivedData.POSITION_SMALL_COUNTER));
        this.addSoundOnSmallBeat = byteToBoolean(buffer.get(ReceivedData.POSITION_ADD_SOUND_ON_SMALL_BEAT));

        byte[] rawBytes = new byte[POSITION_PATH_LEN];
        buffer.position(POSITION_OFF_PATH);
        buffer.get(rawBytes);
        this.selectedSoundPath = parseNullTerminatedString(rawBytes);
    }


    private boolean byteToBoolean(byte b) {
        return b == 1;
    }

    private String parseNullTerminatedString(byte[] bytes) {
        int validLength = 0;
        while (validLength < bytes.length && bytes[validLength] != 0) {
            validLength++;
        }
        return new String(bytes, 0, validLength, StandardCharsets.UTF_8);
    }
}
