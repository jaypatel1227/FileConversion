import * as React from 'react'
import { useEffect, useState } from 'react'
import { ICardProps, CardTable } from "./CardTable";
import "./App.css";

export const API_URL = 'http://localhost:5001/';

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

// interface IConversionReqResponse {
//   file_name?: string;
//   success?: boolean;
//   error?: string;
// }

const buildAdditionalCardInformation = (resp: IServices) => {

  resp.available_services?.forEach((service) => {
    // extract the file extension from the service name
    service.fileExtension = service.name.split('To')[0].toLowerCase();
    // the URL for the post request
    service.postRequestURL = API_URL + 'convert_file/' + service.name + '/';
    // // the function to post the request convert a file
    // service.conversionCallback = async function conversionCallback(): Promise<IConversionReqResponse> {
    //   try {
    //     const data = new FormData();
    //     const file_select = document.querySelector('input[type="file"]');
    //     data.append('file', file_select?.);
    //     const resp = await fetch(, { method: "POST" });
    //     return await resp.json();
    //   }
    //   catch (e) {
    //     return { success: false, error: "Unable to make request or confirm request success." };
    //   }
    // }
  });

  return resp;
}

const AvailableServices: React.FC = () => {
  const [services, setServices] = useState<IServices>({});
  useEffect(() => {
    const fetchData = async () => {
      try {
        const resp = await fetch(API_URL + 'available_options', { method: "GET" });
        let result = await resp.json();
        result = buildAdditionalCardInformation(result); // add the addition properties that we need for the Cards
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
