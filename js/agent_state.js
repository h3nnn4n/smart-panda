const enumValue = (name) => Object.freeze({
    toString: () => name
});

export const AgentState = Object.freeze({
    GAMEOVER: enumValue("AgentState.GAMEOVER"),
    LEARN: enumValue("AgentState.LEARN"),
    SPAWN: enumValue("AgentState.SPAWN"),
    GAMESTART: enumValue("AgentState.GAMESTART"),
    FIND_AND_PLACE: enumValue("AgentState.FIND_AND_PLACE"),
});