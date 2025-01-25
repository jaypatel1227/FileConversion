import { API_URL, IUnavailableServiceParams } from "./App";
import { ICardProps } from "./CardTable";

export interface IServices {
  service_name?: string;
  available_services?: ICardProps[];
  is_unavailable?: boolean;
}

export function buildAdditionalCardInformation(resp: IServices) {
  resp.available_services?.forEach((service) => {
    // extract the file extension from the service name
    service.fileExtension = service.name.split("To")[0].toLowerCase();
    // the URL for the post request
    service.postRequestURL = API_URL + "convert_file/" + service.name + "/";
  });
  return resp;
}

export async function fetchServices(props: IUnavailableServiceParams) {
  try {
    const resp = await fetch(API_URL + "available_options", { method: "GET" });
    let result = await resp.json();
    result = buildAdditionalCardInformation(result); // add the addition properties that we need for the Cards
    props.setServices(result);
  }
  catch (e) {
    props.setServices({ is_unavailable: true });
  }
}

export function findOptions(props: IServices) {

}
