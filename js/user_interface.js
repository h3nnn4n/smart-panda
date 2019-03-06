/* jshint esversion: 6 */

var current_agent_mode;

function radio_mode_change_event() {
    const checked_radio = $("input[name='agent-mode']:checked");
    const unchecked_radio = $("input[name='agent-mode']").not(':checked');
    const new_agent_mode = checked_radio.val();

    if (current_agent_mode != new_agent_mode) {
        // Stuff
    }

    current_agent_mode = new_agent_mode;

    checked_radio.parent().addClass('active');
    unchecked_radio.parent().removeClass('active');
}

export function initInterface() {
    const $radio_mode = $("input[name='agent-mode']");

    $radio_mode.click(radio_mode_change_event);
    radio_mode_change_event();
}

export function useRandomAgent() {
    return current_agent_mode == 'random';
}

export function useLearningAgent() {
    return current_agent_mode == 'learning';
}