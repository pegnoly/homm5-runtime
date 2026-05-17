export type TimelineMessage = {
    timestamp: string,
    message: string
}

export function GetCurrentTimestamp() {
    const now = new Date();

    const date = now.toLocaleDateString('en-GB', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric'
    }).replace(/\//g, ':'); // Convert slashes to colons

    const time = now.toLocaleTimeString('en-GB', {
        hour: '2-digit',
        minute: '2-digit'
    });

    return `[${date} - ${time}]`;
}