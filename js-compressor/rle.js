export function compress(data) {
    const out = [];
    let i = 0;

    while (i < data.length) {
        const byte = data[i];
        let count = 1;

        while (i + count < data.length && data[i + count] === byte && count < 255) {
            count++;
        }

        out.push(byte, count);
        i += count;
    }

    return Buffer.from(out);
}

export function decompress(data) {
    const out = [];

    for (let i = 0; i < data.length; i += 2) {
        const byte = data[i];
        const count = data[i + 1];

        out.push(...Array(count).fill(byte));
    }

    return Buffer.from(out);
}
