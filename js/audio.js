/* 
+++++++++++++++++++++++++++++ Common function ++++++++++++++++++++++++++
*/

export function input_value() {
    const audio = new Audio("assets/select_options.wav");
    audio.play();
}


export function stopFailed() {
    if (failedAudio) {
        failedAudio.pause();
        failedAudio.currentTime = 0;
    }
}

export function failed() {
    const audio = new Audio("assets/failed.wav");
    audio.play();
}

export function select_option() {
    const audio = new Audio("assets/beep.wav");
    audio.play();   
}

export function select_home() {
    const audio = new Audio("assets/home.wav");
    audio.play();   
}

export function clear_input() {
    const audio = new Audio("assets/clear_input.wav");
    audio.play();   
}
