export function compress(data) {
    const str = data.toString(), out = [], win = 20;

    for (let i = 0; i < str.length;) {
        let match = { offset: 0, length: 0 };

        for (let j = Math.max(0, i - win); j < i; j++) {
            let k = 0;
            while (str[j + k] === str[i + k]) k++;
            if (k > match.length) match = { offset: i - j, length: k };
        }

        if (match.length >= 3) {
            out.push(1, match.offset, match.length);
            i += match.length;
        } else {
            out.push(0, str.charCodeAt(i));
            i++;
        }
    }

    return Buffer.from(out);
}

export function decompress(data) {
    const bytes = [...data], out = [];

    for (let i = 0; i < bytes.length;) {
        const flag = bytes[i++];
        if (flag === 0) {
            out.push(String.fromCharCode(bytes[i++]));
        } else {
            const offset = bytes[i++];
            const length = bytes[i++];
            const start = out.length - offset;
            out.push(...out.slice(start, start + length));
        }
    }

    return Buffer.from(out.join(''));
}
