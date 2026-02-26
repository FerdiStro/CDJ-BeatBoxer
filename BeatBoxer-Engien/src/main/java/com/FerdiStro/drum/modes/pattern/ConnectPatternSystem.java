package com.FerdiStro.drum.modes.pattern;

import com.FerdiStro.drum.modes.pattern.objects.PatternMetaData;
import com.FerdiStro.drum.modes.pattern.objects.PatternMetaDataModes;
import com.FerdiStro.drum.modes.pattern.ques.SearchQue;

import java.util.List;

public class ConnectPatternSystem extends AbstractPatternSystem {

    @Override
    PatternMetaDataModes getPatternMetaDataMode() {
        return PatternMetaDataModes.CONNECT;
    }

    @Override
    List<PatternMetaData> specificSearchPattern(SearchQue searchQue) {
        return List.of();
    }


}
