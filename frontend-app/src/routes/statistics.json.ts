import dns from 'dns';

export async function get({ params }) {
    const stats = async () => {

        function value_or_default(value, default_value) {
            return (typeof value === 'undefined') ? default_value : value;
        }

        function is_name_entry(entry) {
            return entry.key === 'name'
        }

        function convert_stats(stats, index) {
            let name = value_or_default(stats.filter(is_name_entry).shift(), { key: 'ignored', value: '' + index }).value;
            let stats_no_name = stats.filter(stat => stat.key !== 'name');
            return {
                name: name,
                stats: stats_no_name
            };
        }

        let stats = new Promise<string[]>((resolve, reject) => {
            dns.resolve('mandrill.lan', "A", (err, resolved_addresses) => {
                if (err) {
                    reject(err)
                    return
                }
                resolve(resolved_addresses);
            })
        }).then(addresses => {
            let promises = addresses.map(address => {
                return fetch('http://' + address + ':9000/api/stats')
                    .then(value => value.json())
                    .then(stats => convert_stats(stats, 0));
            })
            return Promise.all(promises)
        });
        return stats;
    }
    if (stats) {
        return {
            body: await stats().then(value => {
                return {
                    nodes: value
                }
            })
        };
    }
}