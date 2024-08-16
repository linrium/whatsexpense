import "./index.css"
import { useEffect } from "react"
import axios from "axios"

const CLIENT_ID =
  ""

function App() {
  useEffect(() => {
    const data = JSON.stringify({
      email: "dieutrieuphieu96@gmail.com",
      password: "Phieu.123",
    })

    const config = {
      method: "post",
      maxBodyLength: Infinity,
      url: "http://localhost:3000/api/v1/auth/sign-in",
      headers: {
        "Content-Type": "application/json",
      },
      data: data,
    }

    axios
      .request(config)
      .then((response) => {
        console.log(JSON.stringify(response.data))
      })
      .catch((error) => {
        console.log(error)
      })
  }, [])

  return (
    <div>
    </div>
  )
}

export default App
