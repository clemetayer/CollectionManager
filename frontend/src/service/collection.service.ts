import axios from 'axios';
import { ConstantsAPI } from './constantsAPI';

export class CollectionService {

    private readonly axiosInstance = axios.create({
        baseURL: ConstantsAPI.API_PATH
    });

    async listCollections() : Promise<Collection[]> {
        return this.axiosInstance.get<Collection[]>(ConstantsAPI.PATH_COLLECTION_LIST)
            .then((response) => {
                console.log("response = " + response);
                return response.data;
            })
            .catch(error => {
                console.error('(1) error:', error);
                return [];
            })
    }
}