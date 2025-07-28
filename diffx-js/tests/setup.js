// Jest setup file for diffx-js unified API tests

// Global test configuration
jest.setTimeout(30000); // 30 seconds default timeout

// Console configuration for tests
if (process.env.NODE_ENV === 'test') {
    // Suppress console.log in tests unless explicitly needed
    const originalConsoleLog = console.log;
    console.log = (...args) => {
        if (process.env.DEBUG_TESTS) {
            originalConsoleLog(...args);
        }
    };
}

// Global test helpers
global.expectAsync = async (fn) => {
    let error;
    try {
        await fn();
    } catch (e) {
        error = e;
    }
    return expect(() => {
        if (error) throw error;
    });
};

// Error handler for unhandled promise rejections in tests
process.on('unhandledRejection', (reason, promise) => {
    console.error('Unhandled Rejection at:', promise, 'reason:', reason);
});

// Cleanup after tests
afterAll(() => {
    // Any global cleanup needed
});