export async function get({ params }) {
    const stats = async () => {
        return {
            nodes : [
                {
                    name: "Node1",
                    stats: [
                        {
                            key: "stat1",
                            value: Math.random() * 100
                        },
                        {
                            key: "stat2",
                            value: Math.random() * 100
                        },
                    ]
                },
                {
                    name: "OtherNode",
                    stats: [
                        {
                            key: "stat1",
                            value: Math.random() * 100
                        }
                    ]
                }
            ]
        };
    }
    if (stats) {
        return {
            body: await stats()
        };
    }
}