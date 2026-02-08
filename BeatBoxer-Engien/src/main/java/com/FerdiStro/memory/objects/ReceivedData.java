package com.FerdiStro.memory.objects;

import lombok.Getter;
import lombok.ToString;

import java.nio.MappedByteBuffer;

@ToString
public class ReceivedData {
    public static final int POSITION_INCREASE_BPM = 8;
    public static final int POSITION_DECREASE_BPM = 9;
    public static final int POSITION_BECOME_MASTER = 10;

    @Getter
    private final boolean increaseBpm;
    @Getter
    private final boolean decreaseBpm;
    @Getter
    private final boolean becomeMaster;

    public ReceivedData(MappedByteBuffer buffer) {
        this.increaseBpm = byteToBoolean(buffer.get(ReceivedData.POSITION_INCREASE_BPM));
        this.decreaseBpm = byteToBoolean(buffer.get(ReceivedData.POSITION_DECREASE_BPM));
        this.becomeMaster = byteToBoolean(buffer.get(ReceivedData.POSITION_BECOME_MASTER));
    }


    private boolean byteToBoolean(byte b) {
        return b == 1;
    }
}
