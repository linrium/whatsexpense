import axios from "axios";

export async function handler() {
    console.info('updating exchange rates')
    await axios.get(`${process.env.BASE_URL}/api/v1/exchange-rates/latest`)
    console.info('updated exchange rates')
}
