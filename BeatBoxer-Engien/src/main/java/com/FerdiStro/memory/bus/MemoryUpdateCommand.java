package com.FerdiStro.memory.bus;

public enum MemoryUpdateCommand {

    DEFAULT,
    BECOME_MASTER,
    ON_SHOOT_MODUS,
    ON_SHOOT_DIRECT,
    ON_SHOOT_DIRECT_ON_BEAT,
    INCREASE_BPM,
    DECREASE_BPM,
    SMALL_BEAT,
    ADD_BEAT_SMALL,
    REMOVE_BEAT_SMALL,
    EFFECT_ECHO,
    EFFECT_REVERB,
    EFFECT_DISTORTION,
}
