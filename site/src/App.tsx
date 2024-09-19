import React, { SetStateAction } from 'react'
import { useEffect, useState } from 'react'
import { CardTable } from "./CardTable";
import { ISelectorModeParams, SelectorMode } from "./SelectorMode";
import * as Utils from "./Utils";
import "./App.css";

export const API_URL = 'http://localhost:5001/';

function App() {
  return (
    <MainContent />
  )
}

export interface IUnavailableServiceParams {
  setServices: React.Dispatch<SetStateAction<Utils.IServices>>
}

const UnavailableServices: React.FC<IUnavailableServiceParams> = (props) => {
  return (
    <div className='_appHeader _unavaiblableService'>
      <div> The file conversion service appears to be down. Please try again later. </div>
      <button className='_button' onClick={() => props.setServices({})}>Try Again</button>
    </div>
  );
};

const MainContent: React.FC = () => {
  const [services, setServices] = useState<Utils.IServices>({});
  const [searchTerm, setSearchTerm] = useState<string>("");
  const [selectorMode, setSelectorMode] = useState<boolean>(false);
  useEffect(() => {
    Utils.fetchServices({ setServices });
  }, []);

  if (services.is_unavailable) {
    return (
      <UnavailableServices setServices={setServices} />
    );
  }

  return (
    <div>
      <div className='_appHeader'> {services.service_name}</div>
      <HeaderBar searchTerm={searchTerm} outputSearchTerm={setSearchTerm} selectorMode={selectorMode} setSelectorMode={setSelectorMode} setServices={setServices} />
      {selectorMode ? <SelectorMode services={services} /> :
        <CardTable cards={services.available_services ?? []} searchFilter={searchTerm} />
      }
    </div>
  );
}

interface IHeaderBarParams {
  searchTerm: string,
  outputSearchTerm: React.Dispatch<SetStateAction<string>>,
  selectorMode: boolean,
  setSelectorMode: React.Dispatch<SetStateAction<boolean>>,
  setServices: React.Dispatch<SetStateAction<Utils.IServices>>,
}

const HeaderBar: React.FC<IHeaderBarParams> = (props: IHeaderBarParams) => {
  return (
    <div>
      <input type='text' placeholder='🔍' value={props.searchTerm} onChange={(e) => e.isTrusted ? props.outputSearchTerm(e.target.value) : null} name='filter' />
      {!!props.searchTerm ?
        <button onClick={() => props.outputSearchTerm('')} >❌</button>
        : null
      }
      <button className='_button' onClick={() => props.setServices({})}>Clear</button>
      <button className='_button' onClick={() => props.setSelectorMode(!props.selectorMode)}>{props.selectorMode ? 'All Options' : 'Selector Mode'}</button>
    </div>
  );
}

export default App;
