// Start off by initializing a new context.
context = new (window.AudioContext || window.webkitAudioContext)();

function BufferLoader(context, urlList, callback) {
    this.context = context;
    this.urlList = urlList;
    this.onload = callback;
    this.bufferList = new Array();
    this.loadCount = 0;
}

BufferLoader.prototype.loadBuffer = function(url, index) {
    // Load buffer asynchronously
    var request = new XMLHttpRequest();
    request.open('GET', url, true);
    request.responseType = 'arraybuffer';

    var loader = this;

    request.onload = function() {
        // Asynchronously decode the audio file data in request.response
        loader.context.decodeAudioData(
            request.response,
            function(buffer) {
                if (!buffer) {
                    alert('error decoding file data: ' + url);
                    return;
                }
                loader.bufferList[index] = buffer;
                if (++loader.loadCount == loader.urlList.length)
                    loader.onload(loader.bufferList);
            },
            function(error) {
                console.error('decodeAudioData error', error);
            }
        );
    }

    request.onerror = function() {
        alert('BufferLoader: XHR error');
    }

    request.send();
};

BufferLoader.prototype.load = function() {
    for (var i = 0; i < this.urlList.length; ++i)
        this.loadBuffer(this.urlList[i], i);
};

var loader = new BufferLoader(context, ['jump.mp3', 'wall.mp3'], onLoaded);

function onLoaded(buffers) {
    context.buffers = buffers;
};

loader.load();

function play(index, volume) {
    var source = context.createBufferSource();
    var gain = context.createGain();
    gain.gain.value = volume;
    source.buffer = context.buffers[index];
    source.connect(gain);

    var compressor = context.createDynamicsCompressor();
    compressor.threshold.value = 10;
    compressor.ratio.value = 20;
    compressor.reduction.value = -20;
    gain.connect(compressor);
    compressor.connect(context.destination);

    source.start();
}

function play_jump(vol) {
	play(0, vol)
}

function play_wall(vol) {
	play(1, vol)
}
