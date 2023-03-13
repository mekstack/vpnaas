import datetime
import jwt
import redis
from redis.commands.json.path import Path
import config

response_ok= "OK"
# Invalid JWT responses
response_signature_invalid = {"response" : "Failed veryfing jwt signature"}
response_expired = {"response" : "JWT expired"}
response_data_malformed = {"response" : "Data malformed"}
# Invalid actions reponses
response_key_exists = {"response": "Key for this email already exists"}
response_key_doesnt_exist = {"response" : "Key for this email does not exist"}

def verify_jwt_and_get_email(jwt_data):
    try:
        user_data = jwt.decode(jwt_data, config.Config.JWT_SECRET_KEY, algorithms = config.Config.JWT_ALGORITHMS)

    except jwt.exceptions.InvalidSignatureError:
        return response_signature_invalid
    
    try:
        if datetime.datetime.fromtimestamp(user_data["exp"]) < datetime.datetime.utcnow():
            return response_expired

        return {"response" : response_ok, "email" : user_data["email"]}
    
    except KeyError:
        return response_data_malformed

def encode_wrapper(jwt_payload):
    return jwt.encode(jwt_payload, config.Config.JWT_SECRET_KEY, algorithm=config.Config.JWT_ALGORITHMS)

def redis_wrapper():
    return redis.Redis(host=config.Config.REDIS_HOST, port=config.Config.REDIS_PORT,password=config.Config.REDIS_PASS)

def add_key(jwt_data):
    verifier_response = verify_jwt_and_get_email(jwt_data)
    if verifier_response["response"] != response_ok:
        return encode_wrapper(verifier_response)
    
    email = verifier_response["email"]
    # Call wg_core using email. Not implemented yet 
    wg_ip = "0.0.0.0" # not implemented yet
    key = email # not implemented yet
    # Some checks should be here

    redis_con = redis_wrapper()

    if redis_con.exists(email) != 0:
        return encode_wrapper(response_key_exists)

    key_info = {
        "key" : key,
        "timestamp" :  datetime.datetime.now().strftime("%d.%m.%y %H:%M:%S"),
        "ip" : wg_ip,
        "key_state" : "active"
    }

    redis_con.json().set(email, obj=key_info, path=Path.root_path())

    redis_con.close()

    # Call wg core and add key. Not implemented yet    
    
    return encode_wrapper({"response" : response_ok, "key": key})

def get_key(jwt_data):
    verifier_response = verify_jwt_and_get_email(jwt_data)
    if verifier_response["response"] != response_ok:
        return encode_wrapper(verifier_response)
    
    email = verifier_response["email"]

    redis_con = redis_wrapper()
    if redis_con.exists(key) == 0:
        return encode_wrapper(response_key_doesnt_exist)

    key = redis_con.json.get(email, Path(".key"))

    redis_con.close()

    return encode_wrapper({"response" : response_ok, "key": key})



def disable_key(jwt_data):
    verifier_response = verify_jwt_and_get_email(jwt_data)
    if verifier_response["response"] != response_ok:
        return encode_wrapper(verifier_response)
    
    email = verifier_response["email"]
    # Call wg_core using email. Not implemented yet 
    key = email # not implemented yet
    # Some checks should be here

    redis_con = redis_wrapper()
    if redis_con.exists(key) == 0:
        return encode_wrapper(response_key_doesnt_exist)

    key = redis_con.json().get(email, Path(".key"))
    redis_con.json().set(email, obj = "disabled",  path = Path(".state"))
    redis_con.close()

    # call wg core and disable key. Not implemented yet

    return encode_wrapper({"response" : response_ok, "key" : key})




    