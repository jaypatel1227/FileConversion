import * as React from 'react'
import { useEffect, useState } from 'react'
import { ICardProps, CardTable } from "./CardTable";

function App() {

  // <div>
  //   <GetButton />
  //   <PostButton />
  //   <PutButton />
  //   <DeleteButton />
  // </div>
  return (
    <AvailableServices />
  )
}

interface IServices {
  service_name?: string;
  available_services?: ICardProps[];
}


const AvailableServices: React.FC = () => {
  const [services, setServices] = useState<IServices>({});
  useEffect(() => {
    const fetchData = async () => {
      const resp = await fetch('http://localhost:5001/available_options', { method: "GET", });

      if (!resp.ok) {
        throw new Error('Network response was not ok');
      }

      const result = await resp.json();
      setServices(result);
    };
    fetchData();
  }, []);

  return (
    <div>
      <div className='appHeader'> {services.service_name}</div>
      <CardTable cards={services.available_services ?? []} />
    </div>
  );
}

function GetButton() {
  return (
    <button id="getbutton" title='GET' onClick={GetAPICall}> GET REQUEST </button>
  )
}

function PostButton() {
  return (
    <button id="postbutton" title='POST' onClick={PostAPICall}> POST REQUEST </button>
  )
}

function PutButton() {
  return (
    <button id="putbutton" title='PUT'> PUT REQUEST </button>
  )
}

function DeleteButton() {
  return (
    <button id="deletebutton" title='DELETE'> DELETE REQUEST </button>
  )
}

async function GetAPICall() {
  const resp = await fetch('http://localhost:5001/');
  const body = resp.body;
  console.log(resp.text());
  alert(body);
}

function PostAPICall() {
  fetch('http://localhost:5001/echo', {
    method: "POST",
    body: "this is what I want you to echo."
  })
    .then(response => response.text())
    .then(data => console.log(data));
}
export default App;
