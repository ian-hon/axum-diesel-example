import QrScanner from "./qr_code/qr-scanner.min.js";

const video = document.getElementById('qr-video');

const flashToggle = document.getElementById('flash-toggle');

const fileSelector = document.getElementById('file-selector');
// const fileQrResult = document.getElementById('file-qr-result');

function setResult(result) {
    console.log(result.data);
    validateQRCode(result.data);
}

// ####### Web Cam Scanning #######

const scanner = new QrScanner(video, result => setResult(result), {
    onDecodeError: error => {

    },
    highlightScanRegion: true,
    highlightCodeOutline: true,
});

scanner.setInversionMode("both");

// videoContainer.className = e.target.value;
// scanner._updateOverlay(); // reposition the highlight because style 2 sets position: relative

const updateFlashAvailability = () => {
    scanner.hasFlash().then(hasFlash => {
        flashToggle.ariaLabel = hasFlash ? flashToggle.ariaLabel : 'disabled';
    });
};

// scanner.start().then(() => {
//     updateFlashAvailability();
// });

// scanner.stop();

window.scanner = scanner;
window.updateFlashAvailability = updateFlashAvailability;

flashToggle.addEventListener('click', () => {
    if (flashToggle.ariaLabel == 'disabled') {
        return;
    }

    flashToggle.ariaLabel = 'on' ? 'off' : 'on';
    scanner.toggleFlash();
});

// ####### File Scanning #######

fileSelector.addEventListener('change', event => {
    const file = fileSelector.files[0];
    if (!file) {
        return;
    }
    QrScanner.scanImage(file, { returnDetailedScanResult: true })
        .then(result => setResult(result))
        .catch(e => setResult({ data: e || 'No QR code found.' }));
});