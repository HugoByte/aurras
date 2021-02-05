import { main } from '../src';

describe('Main Function Unit Tests', () => {
    test('Can return event params', () => {
        const params = {
            event: { section: "system" }
        };
        console.log(main(params));
        expect(main(params)).toStrictEqual({
            message: `Event received from system`,
            event: { section: "system" }
        })
    });
});