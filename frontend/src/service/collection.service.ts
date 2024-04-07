import axios from 'axios';
import { ConstantsAPI } from './constantsAPI';

export class CollectionService {

    private readonly axiosInstance = axios.create({
        baseURL: ConstantsAPI.API_PATH
    });

    async listCollections() : Promise<Collection[]> {
        return this.axiosInstance.get<Collection[]>(ConstantsAPI.PATH_COLLECTION_LIST)
            .then((response) => {
                console.log("response listing collections : " + response);
                return response.data;
            })
            .catch(error => {
                console.error('error listing collections :', error);
                return [];
            })
    }

    async initCollection(name:String, fromPlaylist?:String) {
        return this.axiosInstance.post(ConstantsAPI.PATH_COLLECTION_INIT, {
                name:name,
                from_playlist:fromPlaylist
            },
            {
                headers: {
                    'Access-Control-Allow-Origin': '*',
                    'Content-Type': 'application/json'
                }
            })
            .catch(error => {
                console.error('error init collection :', error);
            })
    }
}