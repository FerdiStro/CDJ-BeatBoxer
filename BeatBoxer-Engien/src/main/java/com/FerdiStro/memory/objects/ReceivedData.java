package com.FerdiStro.memory.objects;

import lombok.Getter;
import lombok.ToString;

import java.nio.MappedByteBuffer;
import java.nio.charset.StandardCharsets;

@ToString
public class ReceivedData {
    private static final int STRING_LENGTH = 256;

    private static final int POSITION_INCREASE_BPM = 8;
    private static final int POSITION_DECREASE_BPM = 9;
    private static final int POSITION_BECOME_MASTER = 10;

    private static final int POSITION_SMALL_COUNTER = 14;
    private static final int POSITION_ADD_SOUND_ON_SMALL_BEAT = 15;
    private static final int POSITION_START_SELECTED_SOUND_PATH = 16;

    private static final int POSITION_REMOVE_SOUND_ON_SMALL_BEAT = 272;
    private static final int POSITION_START_REMOVE_SOUND_PATH = 280;

    private static final int POSITION_KNOB_VALUE = 536;
    private static final int POSITION_KNOB_ECHO = 537;
    private static final int POSITION_KNOB_REVERB = 538;
    private static final int POSITION_KNOB_DISTORTION = 539;

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

    @Getter
    private final boolean removeSoundOnSmallBeat;
    @Getter
    private final String removeSoundPath;
    @Getter
    private final byte knobValue;
    @Getter
    private final boolean knobEcho;
    @Getter
    private final boolean knobReverb;
    @Getter
    private final boolean knobDistortion;


    public ReceivedData(MappedByteBuffer buffer) {
        this.increaseBpm = byteToBoolean(buffer.get(ReceivedData.POSITION_INCREASE_BPM));
        this.decreaseBpm = byteToBoolean(buffer.get(ReceivedData.POSITION_DECREASE_BPM));
        this.becomeMaster = byteToBoolean(buffer.get(ReceivedData.POSITION_BECOME_MASTER));

        this.smallCounter = Byte.toUnsignedInt(buffer.get(ReceivedData.POSITION_SMALL_COUNTER));
        this.addSoundOnSmallBeat = byteToBoolean(buffer.get(ReceivedData.POSITION_ADD_SOUND_ON_SMALL_BEAT));


        this.selectedSoundPath = byteToString(buffer, POSITION_START_SELECTED_SOUND_PATH);

        this.removeSoundOnSmallBeat = byteToBoolean(buffer.get(POSITION_REMOVE_SOUND_ON_SMALL_BEAT));
        this.removeSoundPath = byteToString(buffer, POSITION_START_REMOVE_SOUND_PATH);

        this.knobValue = buffer.get(POSITION_KNOB_VALUE);
        this.knobEcho = byteToBoolean(buffer.get(POSITION_KNOB_ECHO));
        this.knobReverb = byteToBoolean(buffer.get(POSITION_KNOB_REVERB));
        this.knobDistortion = byteToBoolean(buffer.get(POSITION_KNOB_DISTORTION));

    }

    private String byteToString(MappedByteBuffer buffer, int stringStartPointer) {
        byte[] stringByte = new byte[STRING_LENGTH];
        buffer.position(stringStartPointer);
        buffer.get(stringByte);
        return parseNullTerminatedString(stringByte);
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
