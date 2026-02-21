package com.FerdiStro.drum.command;


import lombok.Getter;

public class DrumCommandObject {
    @Getter
    private final DrumCommand command;

    private final String message;


    @Getter
    private int beatPosition;
    @Getter
    private String fileName;

    @Getter
    private int effectValue;


    public DrumCommandObject(DrumCommand command, int effectValue) {
        if ((command != DrumCommand.EFFECT_ECHO && command != DrumCommand.EFFECT_DISTORTION && command != DrumCommand.EFFECT_REVERB)) {
            this.message = "Wrong constructor or Effect is not implemented. Command will be ignored;";
            this.command = DrumCommand.IGNORE;
            return;
        }
        this.command = command;
        this.message = "Effect on Mixer, (Command: " + command + ") with effect-value: " + effectValue;
        this.effectValue = effectValue;
    }

    public DrumCommandObject(DrumCommand command) {
        if (command != DrumCommand.ON_SHOOT_MODE) {
            this.message = "Wrong constructor!. Command not implemented";
            this.command = DrumCommand.IGNORE;
            return;
        }
        this.command = command;
        this.message = "Default constructor with command: " + command;
    }

    public DrumCommandObject(DrumCommand command, String filePath) {
        if (command != DrumCommand.ON_SHOOT_DIRECT && command != DrumCommand.ON_SHOOT_DIRECT_ON_BEAT || filePath.isBlank()) {
            this.message = "Wrong constructor. Command should be ignored";
            this.command = DrumCommand.IGNORE;
            return;
        }
        this.command = command;
        this.message = "On Shoot (Command: " + command + ") with filePath: " + filePath;
        this.fileName = filePath;
    }


    public DrumCommandObject(DrumCommand command, int beatPosition, String filePath) {
        if (command != DrumCommand.ADD_SOUND && command != DrumCommand.REMOVE_SOUND || filePath.isBlank()) {
            this.message = "Wrong constructor. Command should be ignored";
            this.command = DrumCommand.IGNORE;
            return;
        }
        this.command = command;
        this.message = "Add/Remove sound (Command: " + command + ") with filePath: " + filePath + " to beat " + beatPosition;
        this.beatPosition = beatPosition;
        this.fileName = filePath;
    }


    public String toString() {
        return message;
    }


}
