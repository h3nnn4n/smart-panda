var current_agent_mode;

function radio_mode_change_event() {
    const checked_radio = document.querySelector("input[name='agent-mode']:checked");

    if (!checked_radio) {
        return;
    }

    const new_agent_mode = checked_radio.value;

    if (current_agent_mode != new_agent_mode) {
        // Stuff
    }

    current_agent_mode = new_agent_mode;

    document.querySelectorAll("input[name='agent-mode']").forEach((radio) => {
        radio.parentElement.classList.toggle('active', radio.checked);
    });
}

export function initInterface() {
    document.querySelectorAll("input[name='agent-mode']").forEach((radio) => {
        radio.addEventListener('click', radio_mode_change_event);
    });

    radio_mode_change_event();
}

export function useRandomAgent() {
    return current_agent_mode == 'random';
}

export function useLearningAgent() {
    return current_agent_mode == 'learning';
}
