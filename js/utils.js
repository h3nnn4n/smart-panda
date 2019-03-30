export const shuffle_array = (array) => {
    for (var i = array.length - 1; i > 0; i--) {
        var j = Math.floor(Math.random() * (i + 1));
        var temp = array[i];
        array[i] = array[j];
        array[j] = temp;
    }

    return array;
};

export const suffled_array = (start, length) => {
    let array = Array.from({
        length: length
    }).map((_, index) => (index - start));

    shuffle_array(array);

    return array;
};

export const mean = (values) => {
    if (!values || values.length === 0) {
        return 0;
    }

    const sum = values.reduce((previous, current) => current += previous);
    const avg = sum / values.length;

    return avg;
};

export const rand_float = (min, max) => {
    return Math.random() * (max - min) + min;
};

export const rand_int = (minimum, maximum) => {
    return Math.floor(Math.random() * (maximum - minimum)) + minimum;
};