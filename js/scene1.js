Array.prototype.random = function() {
  return this[Math.round(Math.random() * (this.length - 1))];
};

const success_message_list = ["assets/congratulations-message.wav", "assets/victorymale.wav"]

export function scene1_success() {
  const audioFiles = ["assets/car-collision-success.wav", success_message_list.random()];
  playAudiosSequentially(audioFiles);
}

export function start_running(duration) {
  playSoundForDuration("assets/car-driving.wav", duration)
}

let audioContext;
let audioSource;
let audioElement; // Only needed if using MediaElementSource

export function playSoundForDuration(soundFile, durationSec) {
    // Stop any existing playback
    if (audioSource) {
        audioSource.stop();
        audioSource = null;
    }
    if (audioContext) {
        audioContext.close();
    }

    audioContext = new (window.AudioContext || window.webkitAudioContext)();
    
    // Option 1: If using a file path (fetch + decode)
    fetch(soundFile)
        .then(response => response.arrayBuffer())
        .then(arrayBuffer => audioContext.decodeAudioData(arrayBuffer))
        .then(decodedData => {
            audioSource = audioContext.createBufferSource();
            audioSource.buffer = decodedData;
            audioSource.loop = true;
            audioSource.connect(audioContext.destination);
            audioSource.start();
            
            // Auto-stop after durationSec (optional)
            setTimeout(() => stopAudio(), durationSec * 1000);
        })
        .catch(error => console.error("Error:", error));

    // Option 2: If using an existing Audio element
    // audioElement = new Audio(soundFile);
    // audioSource = audioContext.createMediaElementSource(audioElement);
    // audioSource.connect(audioContext.destination);
    // audioElement.play();
}

export function stop_running() {
    if (audioSource) {
        audioSource.stop(); // Stops BufferSourceNode
        audioSource = null;
    }
    if (audioContext) {
        audioContext.close(); // Releases AudioContext resources
        audioContext = null;
    }
    if (audioElement) {
        audioElement.pause(); // Stops HTMLAudioElement
        audioElement = null;
    }
}

function playAudiosSequentially(audioFiles) {
    let currentIndex = 0;

    function playNext() {
        if (currentIndex >= audioFiles.length) return;

        const audio = new Audio(audioFiles[currentIndex]);
        
        audio.addEventListener('ended', () => {
            currentIndex++;
            playNext();
        });
        
        audio.addEventListener('error', () => {
            console.error(`Failed to load ${audioFiles[currentIndex]}`);
            currentIndex++;
            playNext();
        });
        
        audio.play();
    }

    playNext();
}



/*
async function playSoundForDuration(soundFile, durationSec) {
    try {
        const response = await fetch(soundFile);
        const arrayBuffer = await response.arrayBuffer();
        const audioCtx = new (window.AudioContext || window.webkitAudioContext)();
        
        // Decode the audio data
        const decodedData = await audioCtx.decodeAudioData(arrayBuffer);
        
        // Play in a loop
        source = audioCtx.createBufferSource();
        source.buffer = decodedData;
        source.loop = true;
        source.connect(audioCtx.destination);
        source.start();

        // Stop after `durationSec` seconds
        setTimeout(() => {
            source.stop();
        }, durationSec * 1000);
    } catch (error) {
        console.error("Error playing audio:", error);
    }
}
*/
