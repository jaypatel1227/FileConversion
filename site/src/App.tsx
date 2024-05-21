import * as React from 'react'
import { useEffect, useState } from 'react'
import { ICardProps, CardTable } from "./CardTable";
import "./App.css";

function App() {

  return (
    <AvailableServices />
  )
}

interface IServices {
  service_name?: string;
  available_services?: ICardProps[];
  is_unavailable?: boolean;
}


const AvailableServices: React.FC = () => {
  const [services, setServices] = useState<IServices>({});
  useEffect(() => {
    const fetchData = async () => {
      try {
        const resp = await fetch('http://localhost:5001/available_options', { method: "GET", });
        const result = await resp.json();
        setServices(result);
      }
      catch (e) {
        setServices({ is_unavailable: true });
      }
    };
    fetchData();
  }, []);

  if (services.is_unavailable) {
    return (
      <div className='_appHeader _unavaiblableService'>
        The file conversion service appears to be down. Please try again later.
      </div>
    );
  }

  return (
    <div>
      <div className='_appHeader'> {services.service_name}</div>
      <CardTable cards={services.available_services ?? []} />
    </div>
  );
}

export default App;
