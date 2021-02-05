export function main(params: any) {
    return {
        message: `Event received from ${params.event.section}`,
        ...params
    }
}